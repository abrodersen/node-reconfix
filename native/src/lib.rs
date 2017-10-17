#[macro_use]
extern crate neon;
extern crate reconfix;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsNumber, JsObject, JsFunction, JsNull};
use neon::mem::Handle;
use neon::scope::{Scope, RootScope, ChainedScope};

use std::io;
use std::mem;
use std::result;
use std::error;

use reconfix::{Reconfix, Plugin, FileNode, Content};

struct CallbackPlugin<'a>
{
    scope: &'a mut RootScope<'a>,
    callback: Handle<'a, JsFunction>,
}

impl<'a, 'b> Plugin<'a, 'b, StreamWrapper<'b>> for CallbackPlugin<'a>
{
    fn open(&'a mut self, file: &FileNode) -> result::Result<StreamWrapper<'b>, Box<error::Error + Send + Sync>> {
        let partition = JsNumber::new(self.scope, file.partition.num() as f64);
        let path = &file.path;

        let stream = self.callback.call(self.scope, JsNull::new(), vec![partition])
            .and_then(|v| v.check::<JsObject>())
            .map_err(|e| Box::new(e))?;

        let wrapper = StreamWrapper {
            scope: self.scope,
            stream: stream,
        };

        Ok(wrapper)
    }
}

struct StreamWrapper<'a>
{
    scope: &'a mut RootScope<'a>,
    stream: Handle<'a, JsObject>,
}

impl<'a> io::Read for StreamWrapper<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl<'a> io::Write for StreamWrapper<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> Content for StreamWrapper<'a> {}

pub struct ReconfixWrapper {
    reconfix: Reconfix,
}

declare_types! {
    pub class JsReconfix for ReconfixWrapper {
        init(call) {
            let scope = call.scope;
            // let open = call.arguments.require(scope, 0)?.check::<JsFunction>()?;
            // let escaped = scope.chained(move |&mut scope| {
            //     scope.escape(open)
            // });
            // let plugin = CallbackPlugin {
            //     scope: None,
            //     callback: escaped,
            // };

            let reconfix = Reconfix::new();

            Ok(ReconfixWrapper {
                //plugin: plugin,
                reconfix: reconfix,
            })
        }
    }
}

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});
