#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::case::{
    Case,
    CaseRef,
};

#[ink::contract]
pub mod case {
    use ink_prelude:: {
        string::String,
        vec::Vec,
        collections::BTreeMap,
    };
    use scale::{
        Decode,
        Encode,
    };

    pub type Id = u32;

    #[ink(storage)]
    pub struct Case {
        pub case: BTreeMap<Id, CaseNFT>,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CaseNFT {
        pub title: String,
        pub description: String,
        pub category: Category,
        pub owner: AccountId,
        pub bounty: Balance,
        pub file: Hash,
        pub status: Status,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct CaseNFTOutput {
        pub case_id: Id,
        pub title: String,
        pub description: String,
        pub category: Category,
        pub owner: AccountId,
        pub bounty: Balance,
        pub file: Hash,
        pub status: Status,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Category {
        Scam,
        Web,
        Person,
        ConspiracyTheory,
        Others,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Status {
        New,
        Evidence,
        Voting,
        Close,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        CaseNotFound,
    }

    impl CaseNFTOutput {
        fn get_case(case_id: Id, case: &CaseNFT) -> CaseNFTOutput {
            CaseNFTOutput {
                case_id: case_id.clone(),
                title: case.title.clone(),
                description: case.description.clone(),
                category: case.category.clone(),
                owner: case.owner.clone(),
                bounty: case.bounty.clone(),
                file: case.file.clone(),
                status: case.status.clone(),
            }
        }
    }

    impl Case {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {
                case: BTreeMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_case(&mut self, case: CaseNFT) {
            let length: Id = (self.case.len() as Id).checked_add(1).unwrap();
            self.case.insert(length, case);
        }

        #[ink(message)]
        pub fn burn_case(&mut self, case_id: Id) -> Result<(), Error> {
            if !self.case.contains_key(&case_id) {
                return Err(Error::CaseNotFound)
            };
            self.case.remove(&case_id);
            Ok(())
        }

        #[ink(message)]
        pub fn update_case(&mut self, case_id: Id, new_case: CaseNFT) -> Result<(), Error> {
            let case: &mut CaseNFT = self
                .case
                .get_mut(&case_id)
                .ok_or(Error::CaseNotFound)?;
            *case = new_case;
            Ok(())
        }

        #[ink(message)]
        pub fn get_case_by_id(&self, case_id: Id) -> Option<CaseNFTOutput> {
            if let Some(case) = self.case.get(&case_id) {
                let case: CaseNFTOutput = CaseNFTOutput::get_case(case_id, case);
                Some(case)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_case(&self) -> Vec<CaseNFTOutput> {
            let case: Vec<CaseNFTOutput> = self
                .case
                .iter()
                .map(|(case_id, case)| CaseNFTOutput::get_case(*case_id, case))
                .collect();
            case
        }

        #[ink(message)]
        pub fn get_case_title(&self, case_id: Id) -> Option<String> {
            if let Some(case) = self.case.get(&case_id) {
                let case_title: String = case.title.clone();
                Some(case_title)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_case_id(&self, case_id: Id) -> Id {
            if self.case.get(&case_id).is_some() {
                case_id
            } else {
                0 as u32
            }
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: Hash) {
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
    }
}
