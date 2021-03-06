/// Implementation of the _thread module, currently noop implementation as RustPython doesn't yet
/// support threading
use crate::function::PyFuncArgs;
use crate::pyobject::{PyObjectRef, PyResult};
use crate::vm::VirtualMachine;

#[cfg(not(target_os = "windows"))]
const PY_TIMEOUT_MAX: isize = std::isize::MAX;

#[cfg(target_os = "windows")]
const PY_TIMEOUT_MAX: isize = 0xffffffff * 1_000_000;

const TIMEOUT_MAX: f64 = (PY_TIMEOUT_MAX / 1_000_000_000) as f64;

fn rlock_acquire(vm: &VirtualMachine, _args: PyFuncArgs) -> PyResult {
    Ok(vm.get_none())
}

fn rlock_release(_zelf: PyObjectRef) {}

fn rlock_enter(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(instance, None)]);
    Ok(instance.clone())
}

fn rlock_exit(
    // The context manager protocol requires these, but we don't use them
    _instance: PyObjectRef,
    _exception_type: PyObjectRef,
    _exception_value: PyObjectRef,
    _traceback: PyObjectRef,
    vm: &VirtualMachine,
) -> PyResult {
    Ok(vm.get_none())
}

fn get_ident(_vm: &VirtualMachine) -> u32 {
    1
}

fn allocate_lock(vm: &VirtualMachine) -> PyResult {
    let lock_class = vm.class("_thread", "RLock");
    vm.invoke(&lock_class.into_object(), vec![])
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;

    let rlock_type = py_class!(ctx, "_thread.RLock", ctx.object(), {
        "acquire" => ctx.new_method(rlock_acquire),
        "release" => ctx.new_method(rlock_release),
        "__enter__" => ctx.new_method(rlock_enter),
        "__exit__" => ctx.new_method(rlock_exit),
    });

    py_module!(vm, "_thread", {
        "RLock" => rlock_type,
        "get_ident" => ctx.new_function(get_ident),
        "allocate_lock" => ctx.new_function(allocate_lock),
        "TIMEOUT_MAX" => ctx.new_float(TIMEOUT_MAX),
    })
}
