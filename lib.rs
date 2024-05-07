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
        pub contract_owner: AccountId,
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

    #[derive(Encode, Decode, Debug, Clone, PartialEq, Copy)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Category {
        Scam,
        Web,
        Person,
        ConspiracyTheory,
        Others,
        All,
    }

    #[derive(Encode, Decode, Debug, Clone, PartialEq, Copy)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Status {
        New,
        Evidence,
        Voting,
        Close,
        All,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        CaseNotFound,
        Unauthorized,
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
        pub fn new(contract_owner: AccountId) -> Self {
            Self {
                case: BTreeMap::new(),
                contract_owner,
            }
        }

        #[ink(message)]
        pub fn set_case(&mut self, case: CaseNFT) {
            let last_case: &u32 = match self.case.last_key_value() {
                Some(data) => data.0,
                None => &0,
            };
            let last_id: u32 = last_case.checked_add(1).unwrap();
            self.case.insert(last_id, case);
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

        // Pagination
        // Get cases which match the page number,
        // entries, keyword, category, and status.
        // Return data is (vec, total cases).
        #[ink(message)]
        pub fn get_all_case(
            &self, 
            page: Id, 
            entry: Id, 
            keyword: String, 
            category: Category, 
            status: Status
        ) -> (Vec<CaseNFTOutput>, Id) {
            let mut filtered_cases: Vec<CaseNFTOutput> = Vec::new();
            let mut total_cases: Id = 0;
            let page = if page < 1 { 1 } else { page };
            let start_index = (page - 1) * entry;
            let end_index = start_index + entry;
            for (case_id, case) in self.case.range(..) {
                if Self::case_contains_keywords(case, &keyword)
                    && Self::category_matches(case, category)
                    && Self::status_matches(case, status)
                {
                    total_cases += 1;
                    if total_cases > start_index && total_cases <= end_index {
                        filtered_cases.push(CaseNFTOutput::get_case(*case_id, case));
                    }
                }
            }
            (filtered_cases, total_cases)
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
            let caller = self.env().caller();
            if caller != self.contract_owner {
                return Err(Error::Unauthorized);
            }
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }

        fn case_contains_keywords(case: &CaseNFT, keyword: &String) -> bool {
            case.title.contains(keyword) || case.description.contains(keyword)
        }
        
        fn category_matches(case: &CaseNFT, category: Category) -> bool {
            match category {
                Category::Scam | Category::Web | Category::Person | Category::ConspiracyTheory | Category::Others => {
                    case.category == category
                },
                Category::All => true
            }
        }
        
        fn status_matches(case: &CaseNFT, status: Status) -> bool {
            match status {
                Status::New | Status::Evidence | Status::Voting | Status::Close => {
                    case.status == status
                },
                Status::All => true
            }
        }
    }
}
