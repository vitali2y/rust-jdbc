use java;
use java::lang::ClassLoader;
use java::lang::IObject;
use java::ToValue;
use jvm;
use std;

jvm_object!(Thread, "java/lang/Thread");

impl Thread {
  pub fn current_thread() -> java::Result<Thread> {
    return jvm_call!(static, nonnull Thread: "currentThread", "()Ljava/lang/Thread;", &[]);
  }

  pub fn set_context_class_loader(&self, class_loader: &ClassLoader) -> java::Result<()> {
    return jvm_call!(
      void: self,
      "setContextClassLoader",
      "(Ljava/lang/ClassLoader;)V",
      &[&class_loader.to_value()]
    );
  }
}
