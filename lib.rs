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

    pub type CaseId = u32;

    #[ink(storage)]
    pub struct Case {
        pub case: BTreeMap<CaseId, CaseNFT>,
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

    impl CaseNFT {
        fn get_case(case: &CaseNFT) -> CaseNFT {
            CaseNFT {
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
            let length = (self.case.len() as u32).checked_add(1).unwrap();
            self.case.insert(length, case);
        }

        #[ink(message)]
        pub fn get_case_by_id(&self, case_id: CaseId) -> Option<CaseNFT> {
            if let Some(case) = self.case.get(&case_id) {
                let case = CaseNFT::get_case(case);
                Some(case)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_case(&self) -> Vec<CaseNFT> {
            let case = self
                .case
                .iter()
                .map(|(_id, case)| CaseNFT::get_case(case))
                .collect();
            case
        }

        #[ink(message)]
        pub fn get_case_id(&self, case_id: CaseId) -> CaseId {
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
