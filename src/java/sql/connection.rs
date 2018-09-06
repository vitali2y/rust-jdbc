use java;
use java::lang::IObject;
use java::sql::Statement;
use jvm;
use std;

jvm_object!(Connection, "java/sql/Connection");

impl Connection {
  pub fn create_statement(&self) -> java::Result<Statement> {
    return jvm_call!(nonnull Statement: self, "createStatement", "()Ljava/sql/Statement;", &[]);
  }

  pub fn close(&self) {
    jvm_call!(void: self, "close", "()V", &[]);
  }
}
