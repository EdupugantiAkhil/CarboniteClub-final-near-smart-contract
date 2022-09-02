use crate::*;

impl Contract {
    /// adds given token_metadata and associated a/c ID to tokens_by_account_id, panics if given a/c ID was already present in the collection
    pub fn internal_add_token_to_owner(
        &mut self,
        owner_id: &AccountId,
        token_metadata: &TokenMetadata,
    ) {
        require!(
            self.tokens_by_account_id
                .insert(owner_id, token_metadata)
                .is_none()
                == true,
            "account ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );
    }

    /// adds given company and associated a/c ID to whitelisted_companies, panics if given a/c ID was already present in the collection
    pub fn internal_add_company_to_whitelisted_companies(
        &mut self,
        company_id: &AccountId,
        company: &Company,
    ) {
        require!(
            self.whitelisted_companies
                .insert(company_id, company)
                .is_none()
                == true,
            "company ID already exists" // would never reach this since it will fail at sub account creation itself but still for security reasons
        );
    }

    /// adds the task_id associated with company_id to collection and panics if task_id already exists
    pub fn internal_add_tasks_to_company(&mut self, company_id: &AccountId, task_id: &TaskId) {
        let mut task_set = self.tasks_by_company.get(&company_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::TasksByCompanyInner {
                company_id_hash: hash_account_id(company_id),
            })
        });

        require!(
            task_set.insert(task_id),
            format!("{company_id} already has task {task_id}")
        );

        self.tasks_by_company.insert(company_id, &task_set);
    }

    /// adds the task_id associated with company_id to collection and panics if task_id already exists
    pub fn internal_add_tasks_to_account(&mut self, user_id: &AccountId, task_id: &TaskId) {
        let mut task_set = self
            .tasks_completed_per_account
            .get(&user_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::TasksCompletedPerAccountInner {
                    account_id_hash: hash_account_id(user_id),
                })
            });

        require!(
            task_set.insert(task_id),
            format!("{user_id} already has completed {task_id}")
        );

        self.tasks_completed_per_account.insert(user_id, &task_set);
    }
}
