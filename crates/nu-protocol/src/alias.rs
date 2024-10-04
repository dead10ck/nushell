use crate::{
    ast::Expression,
    engine::{Call, Command, CommandType, EngineState, Stack},
    PipelineData, ShellError, Signature,
};
use std::fmt::Debug;

/// Command wrapper of an alias.
///
/// Our current aliases are implemented as wrapping commands
/// This has some limitations compared to text-substitution macro aliases but can reliably use more
/// of our machinery
#[derive(Clone)]
pub struct Alias {
    pub name: String,
    /// Wrapped inner [`Command`]. `None` if alias of external call
    pub command: Option<Box<dyn Command>>,
    pub wrapped_call: Expression,
    pub usage: String,
    pub extra_usage: String,
}

impl Command for Alias {
    fn name(&self) -> &str {
        &self.name
    }

    fn signature(&self) -> Signature {
        if let Some(cmd) = &self.command {
            cmd.signature()
        } else {
            Signature::new(&self.name).allows_unknown_args()
        }
    }

    fn usage(&self) -> &str {
        &self.usage
    }

    fn extra_usage(&self) -> &str {
        &self.extra_usage
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Err(ShellError::NushellFailedSpanned {
            msg: "Can't run alias directly. Unwrap it first".to_string(),
            label: "originates from here".to_string(),
            span: call.head,
        })
    }

    fn command_type(&self) -> CommandType {
        CommandType::Alias
    }

    fn as_alias(&self) -> Option<&Alias> {
        Some(self)
    }
}

impl Debug for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Alias")
            .field("name", &self.name)
            .field("wrapped_call", &self.wrapped_call)
            .field("usage", &self.usage)
            .field("extra_usage", &self.extra_usage)
            .finish_non_exhaustive()
    }
}
