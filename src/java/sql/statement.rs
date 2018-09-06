use java;
use java::lang::IObject;
use java::sql::ResultSet;
use java::ToValue;
use jvm;
use std;

jvm_object!(Statement, "java/sql/Statement");

impl Statement {
  pub fn execute(&self, sql: &str) -> java::Result<bool> {
    let sql = jvm::String::from_str(sql);

    return jvm_call!(
      bool: self,
      "execute",
      "(Ljava/lang/String;)Z",
      &[&sql.to_value()]
    );
  }

  pub fn execute_query(&self, sql: &str) -> java::Result<ResultSet> {
    let sql = jvm::String::from_str(sql);

    return jvm_call!(nonnull ResultSet: self, "executeQuery", "(Ljava/lang/String;)Ljava/sql/ResultSet;", &[&sql.to_value()]);
  }

  pub fn execute_update(&self, sql: &str) -> java::Result<i32> {
    let sql = jvm::String::from_str(sql);

    return jvm_call!(
      int: self,
      "executeUpdate",
      "(Ljava/lang/String;I)I",
      &[&sql.to_value(), &1.to_value()]
    );
  }

  pub fn get_generated_keys(&self) -> java::Result<ResultSet> {
    return jvm_call!(nonnull ResultSet: self, "getGeneratedKeys", "()Ljava/sql/ResultSet;", &[]);
  }

  pub fn close(&self) {
    jvm_call!(void: self, "close", "()V", &[]);
  }
}
