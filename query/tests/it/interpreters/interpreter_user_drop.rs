// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_base::tokio;
use common_exception::Result;
use common_meta_types::PasswordType;
use common_meta_types::UserInfo;
use databend_query::interpreters::*;
use databend_query::sql::*;
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_drop_user_interpreter() -> Result<()> {
    common_tracing::init_default_ut_tracing();

    let ctx = crate::tests::create_query_context()?;
    let tenant = ctx.get_tenant();

    {
        static TEST_QUERY: &str = "DROP USER 'test'@'localhost'";
        let plan = PlanParser::parse(TEST_QUERY, ctx.clone()).await?;
        let executor = InterpreterFactory::get(ctx.clone(), plan.clone())?;
        assert_eq!(executor.name(), "DropUserInterpreter");
        let ret = executor.execute(None).await;
        assert!(ret.is_err())
    }

    {
        static TEST_QUERY: &str = "DROP USER IF EXISTS 'test'@'localhost'";
        let plan = PlanParser::parse(TEST_QUERY, ctx.clone()).await?;
        let executor = InterpreterFactory::get(ctx.clone(), plan.clone())?;
        assert_eq!(executor.name(), "DropUserInterpreter");
        let ret = executor.execute(None).await;
        assert!(ret.is_ok())
    }

    {
        let name = "test";
        let hostname = "localhost";
        let password = "test";
        let user_info = UserInfo::new(
            name.to_string(),
            hostname.to_string(),
            Vec::from(password),
            PasswordType::PlainText,
        );
        let user_mgr = ctx.get_user_manager();
        user_mgr.add_user(&tenant, user_info).await?;

        let old_user = user_mgr.get_user(&tenant, name, hostname).await?;
        assert_eq!(old_user.password, Vec::from(password));

        static TEST_QUERY: &str = "DROP USER 'test'@'localhost'";
        let plan = PlanParser::parse(TEST_QUERY, ctx.clone()).await?;
        let executor = InterpreterFactory::get(ctx, plan.clone())?;
        assert_eq!(executor.name(), "DropUserInterpreter");
        executor.execute(None).await?;
    }

    Ok(())
}
