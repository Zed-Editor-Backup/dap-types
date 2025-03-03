// This file is autogenerated. Do not edit by hand.
// To regenerate from schema, run `cargo run -p generator`.

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// Request is a request, with associated command, and argument and response types.
pub trait Request {
    const COMMAND: &'static str;
    type Arguments: Debug + Clone + Serialize + DeserializeOwned + Send + Sync;
    type Response: Debug + Clone + Serialize + DeserializeOwned + Send + Sync;
}

/// The `cancel` request is used by the client in two situations:
/// - to indicate that it is no longer interested in the result produced by a specific request issued earlier
/// - to cancel a progress sequence.
/// Clients should only call this request if the corresponding capability `supportsCancelRequest` is true.
/// This request has a hint characteristic: a debug adapter can only be expected to make a 'best effort' in honoring this request but there are no guarantees.
/// The `cancel` request may return an error if it could not cancel an operation but a client should refrain from presenting this error to end users.
/// The request that got cancelled still needs to send a response back. This can either be a normal result (`success` attribute true) or an error response (`success` attribute false and the `message` set to `cancelled`).
/// Returning partial results from a cancelled request is possible but please note that a client has no generic way for detecting that a response is partial or not.
/// The progress that got cancelled still needs to send a `progressEnd` event back.
///  A client should not assume that progress just got cancelled after sending the `cancel` request.
pub enum Cancel {}

impl Request for Cancel {
    const COMMAND: &'static str = "cancel";
    type Arguments = crate::CancelArguments;
    type Response = ();
}

/// This request is sent from the debug adapter to the client to run a command in a terminal.
/// This is typically used to launch the debuggee in a terminal provided by the client.
/// This request should only be called if the corresponding client capability `supportsRunInTerminalRequest` is true.
/// Client implementations of `runInTerminal` are free to run the command however they choose including issuing the command to a command line interpreter (aka 'shell'). Argument strings passed to the `runInTerminal` request must arrive verbatim in the command to be run. As a consequence, clients which use a shell are responsible for escaping any special shell characters in the argument strings to prevent them from being interpreted (and modified) by the shell.
/// Some users may wish to take advantage of shell processing in the argument strings. For clients which implement `runInTerminal` using an intermediary shell, the `argsCanBeInterpretedByShell` property can be set to true. In this case the client is requested not to escape any special shell characters in the argument strings.
pub enum RunInTerminal {}

impl Request for RunInTerminal {
    const COMMAND: &'static str = "runInTerminal";
    type Arguments = crate::RunInTerminalRequestArguments;
    type Response = crate::RunInTerminalResponse;
}

/// This request is sent from the debug adapter to the client to start a new debug session of the same type as the caller.
/// This request should only be sent if the corresponding client capability `supportsStartDebuggingRequest` is true.
/// A client implementation of `startDebugging` should start a new debug session (of the same type as the caller) in the same way that the caller's session was started. If the client supports hierarchical debug sessions, the newly created session can be treated as a child of the caller session.
pub enum StartDebugging {}

impl Request for StartDebugging {
    const COMMAND: &'static str = "startDebugging";
    type Arguments = crate::StartDebuggingRequestArguments;
    type Response = ();
}

/// The `initialize` request is sent as the first request from the client to the debug adapter in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.
/// Until the debug adapter has responded with an `initialize` response, the client must not send any additional requests or events to the debug adapter.
/// In addition the debug adapter is not allowed to send any requests or events to the client until it has responded with an `initialize` response.
/// The `initialize` request may only be sent once.
pub enum Initialize {}

impl Request for Initialize {
    const COMMAND: &'static str = "initialize";
    type Arguments = crate::InitializeRequestArguments;
    type Response = crate::Capabilities;
}

/// This request indicates that the client has finished initialization of the debug adapter.
/// So it is the last request in the sequence of configuration requests (which was started by the `initialized` event).
/// Clients should only call this request if the corresponding capability `supportsConfigurationDoneRequest` is true.
pub enum ConfigurationDone {}

impl Request for ConfigurationDone {
    const COMMAND: &'static str = "configurationDone";
    type Arguments = crate::ConfigurationDoneArguments;
    type Response = ();
}

/// This launch request is sent from the client to the debug adapter to start the debuggee with or without debugging (if `noDebug` is true).
/// Since launching is debugger/runtime specific, the arguments for this request are not part of this specification.
pub enum Launch {}

impl Request for Launch {
    const COMMAND: &'static str = "launch";
    type Arguments = crate::LaunchRequestArguments;
    type Response = ();
}

/// The `attach` request is sent from the client to the debug adapter to attach to a debuggee that is already running.
/// Since attaching is debugger/runtime specific, the arguments for this request are not part of this specification.
pub enum Attach {}

impl Request for Attach {
    const COMMAND: &'static str = "attach";
    type Arguments = crate::AttachRequestArguments;
    type Response = ();
}

/// Restarts a debug session. Clients should only call this request if the corresponding capability `supportsRestartRequest` is true.
/// If the capability is missing or has the value false, a typical client emulates `restart` by terminating the debug adapter first and then launching it anew.
pub enum Restart {}

impl Request for Restart {
    const COMMAND: &'static str = "restart";
    type Arguments = crate::RestartArguments;
    type Response = ();
}

/// The `disconnect` request asks the debug adapter to disconnect from the debuggee (thus ending the debug session) and then to shut down itself (the debug adapter).
/// In addition, the debug adapter must terminate the debuggee if it was started with the `launch` request. If an `attach` request was used to connect to the debuggee, then the debug adapter must not terminate the debuggee.
/// This implicit behavior of when to terminate the debuggee can be overridden with the `terminateDebuggee` argument (which is only supported by a debug adapter if the corresponding capability `supportTerminateDebuggee` is true).
pub enum Disconnect {}

impl Request for Disconnect {
    const COMMAND: &'static str = "disconnect";
    type Arguments = crate::DisconnectArguments;
    type Response = ();
}

/// The `terminate` request is sent from the client to the debug adapter in order to shut down the debuggee gracefully. Clients should only call this request if the capability `supportsTerminateRequest` is true.
/// Typically a debug adapter implements `terminate` by sending a software signal which the debuggee intercepts in order to clean things up properly before terminating itself.
/// Please note that this request does not directly affect the state of the debug session: if the debuggee decides to veto the graceful shutdown for any reason by not terminating itself, then the debug session just continues.
/// Clients can surface the `terminate` request as an explicit command or they can integrate it into a two stage Stop command that first sends `terminate` to request a graceful shutdown, and if that fails uses `disconnect` for a forceful shutdown.
pub enum Terminate {}

impl Request for Terminate {
    const COMMAND: &'static str = "terminate";
    type Arguments = crate::TerminateArguments;
    type Response = ();
}

/// The `breakpointLocations` request returns all possible locations for source breakpoints in a given range.
/// Clients should only call this request if the corresponding capability `supportsBreakpointLocationsRequest` is true.
pub enum BreakpointLocations {}

impl Request for BreakpointLocations {
    const COMMAND: &'static str = "breakpointLocations";
    type Arguments = crate::BreakpointLocationsArguments;
    type Response = crate::BreakpointLocationsResponse;
}

/// Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.
/// To clear all breakpoint for a source, specify an empty array.
/// When a breakpoint is hit, a `stopped` event (with reason `breakpoint`) is generated.
pub enum SetBreakpoints {}

impl Request for SetBreakpoints {
    const COMMAND: &'static str = "setBreakpoints";
    type Arguments = crate::SetBreakpointsArguments;
    type Response = crate::SetBreakpointsResponse;
}

/// Replaces all existing function breakpoints with new function breakpoints.
/// To clear all function breakpoints, specify an empty array.
/// When a function breakpoint is hit, a `stopped` event (with reason `function breakpoint`) is generated.
/// Clients should only call this request if the corresponding capability `supportsFunctionBreakpoints` is true.
pub enum SetFunctionBreakpoints {}

impl Request for SetFunctionBreakpoints {
    const COMMAND: &'static str = "setFunctionBreakpoints";
    type Arguments = crate::SetFunctionBreakpointsArguments;
    type Response = crate::SetFunctionBreakpointsResponse;
}

/// The request configures the debugger's response to thrown exceptions. Each of the `filters`, `filterOptions`, and `exceptionOptions` in the request are independent configurations to a debug adapter indicating a kind of exception to catch. An exception thrown in a program should result in a `stopped` event from the debug adapter (with reason `exception`) if any of the configured filters match.
/// Clients should only call this request if the corresponding capability `exceptionBreakpointFilters` returns one or more filters.
pub enum SetExceptionBreakpoints {}

impl Request for SetExceptionBreakpoints {
    const COMMAND: &'static str = "setExceptionBreakpoints";
    type Arguments = crate::SetExceptionBreakpointsArguments;
    type Response = crate::SetExceptionBreakpointsResponse;
}

/// Obtains information on a possible data breakpoint that could be set on an expression or variable.
/// Clients should only call this request if the corresponding capability `supportsDataBreakpoints` is true.
pub enum DataBreakpointInfo {}

impl Request for DataBreakpointInfo {
    const COMMAND: &'static str = "dataBreakpointInfo";
    type Arguments = crate::DataBreakpointInfoArguments;
    type Response = crate::DataBreakpointInfoResponse;
}

/// Replaces all existing data breakpoints with new data breakpoints.
/// To clear all data breakpoints, specify an empty array.
/// When a data breakpoint is hit, a `stopped` event (with reason `data breakpoint`) is generated.
/// Clients should only call this request if the corresponding capability `supportsDataBreakpoints` is true.
pub enum SetDataBreakpoints {}

impl Request for SetDataBreakpoints {
    const COMMAND: &'static str = "setDataBreakpoints";
    type Arguments = crate::SetDataBreakpointsArguments;
    type Response = crate::SetDataBreakpointsResponse;
}

/// Replaces all existing instruction breakpoints. Typically, instruction breakpoints would be set from a disassembly window.
/// To clear all instruction breakpoints, specify an empty array.
/// When an instruction breakpoint is hit, a `stopped` event (with reason `instruction breakpoint`) is generated.
/// Clients should only call this request if the corresponding capability `supportsInstructionBreakpoints` is true.
pub enum SetInstructionBreakpoints {}

impl Request for SetInstructionBreakpoints {
    const COMMAND: &'static str = "setInstructionBreakpoints";
    type Arguments = crate::SetInstructionBreakpointsArguments;
    type Response = crate::SetInstructionBreakpointsResponse;
}

/// The request resumes execution of all threads. If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true resumes only the specified thread. If not all threads were resumed, the `allThreadsContinued` attribute of the response should be set to false.
pub enum Continue {}

impl Request for Continue {
    const COMMAND: &'static str = "continue";
    type Arguments = crate::ContinueArguments;
    type Response = crate::ContinueResponse;
}

/// The request executes one step (in the given granularity) for the specified thread and allows all other threads to run freely by resuming them.
/// If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.
/// The debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.
pub enum Next {}

impl Request for Next {
    const COMMAND: &'static str = "next";
    type Arguments = crate::NextArguments;
    type Response = ();
}

/// The request resumes the given thread to step into a function/method and allows all other threads to run freely by resuming them.
/// If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.
/// If the request cannot step into a target, `stepIn` behaves like the `next` request.
/// The debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.
/// If there are multiple function/method calls (or other targets) on the source line,
/// the argument `targetId` can be used to control into which target the `stepIn` should occur.
/// The list of possible targets for a given source line can be retrieved via the `stepInTargets` request.
pub enum StepIn {}

impl Request for StepIn {
    const COMMAND: &'static str = "stepIn";
    type Arguments = crate::StepInArguments;
    type Response = ();
}

/// The request resumes the given thread to step out (return) from a function/method and allows all other threads to run freely by resuming them.
/// If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.
/// The debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.
pub enum StepOut {}

impl Request for StepOut {
    const COMMAND: &'static str = "stepOut";
    type Arguments = crate::StepOutArguments;
    type Response = ();
}

/// The request executes one backward step (in the given granularity) for the specified thread and allows all other threads to run backward freely by resuming them.
/// If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.
/// The debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.
/// Clients should only call this request if the corresponding capability `supportsStepBack` is true.
pub enum StepBack {}

impl Request for StepBack {
    const COMMAND: &'static str = "stepBack";
    type Arguments = crate::StepBackArguments;
    type Response = ();
}

/// The request resumes backward execution of all threads. If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true resumes only the specified thread. If not all threads were resumed, the `allThreadsContinued` attribute of the response should be set to false.
/// Clients should only call this request if the corresponding capability `supportsStepBack` is true.
pub enum ReverseContinue {}

impl Request for ReverseContinue {
    const COMMAND: &'static str = "reverseContinue";
    type Arguments = crate::ReverseContinueArguments;
    type Response = ();
}

/// The request restarts execution of the specified stack frame.
/// The debug adapter first sends the response and then a `stopped` event (with reason `restart`) after the restart has completed.
/// Clients should only call this request if the corresponding capability `supportsRestartFrame` is true.
pub enum RestartFrame {}

impl Request for RestartFrame {
    const COMMAND: &'static str = "restartFrame";
    type Arguments = crate::RestartFrameArguments;
    type Response = ();
}

/// The request sets the location where the debuggee will continue to run.
/// This makes it possible to skip the execution of code or to execute code again.
/// The code between the current location and the goto target is not executed but skipped.
/// The debug adapter first sends the response and then a `stopped` event with reason `goto`.
/// Clients should only call this request if the corresponding capability `supportsGotoTargetsRequest` is true (because only then goto targets exist that can be passed as arguments).
pub enum Goto {}

impl Request for Goto {
    const COMMAND: &'static str = "goto";
    type Arguments = crate::GotoArguments;
    type Response = ();
}

/// The request suspends the debuggee.
/// The debug adapter first sends the response and then a `stopped` event (with reason `pause`) after the thread has been paused successfully.
pub enum Pause {}

impl Request for Pause {
    const COMMAND: &'static str = "pause";
    type Arguments = crate::PauseArguments;
    type Response = ();
}

/// The request returns a stacktrace from the current execution state of a given thread.
/// A client can request all stack frames by omitting the startFrame and levels arguments. For performance-conscious clients and if the corresponding capability `supportsDelayedStackTraceLoading` is true, stack frames can be retrieved in a piecemeal way with the `startFrame` and `levels` arguments. The response of the `stackTrace` request may contain a `totalFrames` property that hints at the total number of frames in the stack. If a client needs this total number upfront, it can issue a request for a single (first) frame and depending on the value of `totalFrames` decide how to proceed. In any case a client should be prepared to receive fewer frames than requested, which is an indication that the end of the stack has been reached.
pub enum StackTrace {}

impl Request for StackTrace {
    const COMMAND: &'static str = "stackTrace";
    type Arguments = crate::StackTraceArguments;
    type Response = crate::StackTraceResponse;
}

/// The request returns the variable scopes for a given stack frame ID.
pub enum Scopes {}

impl Request for Scopes {
    const COMMAND: &'static str = "scopes";
    type Arguments = crate::ScopesArguments;
    type Response = crate::ScopesResponse;
}

/// Retrieves all child variables for the given variable reference.
/// A filter can be used to limit the fetched children to either named or indexed children.
pub enum Variables {}

impl Request for Variables {
    const COMMAND: &'static str = "variables";
    type Arguments = crate::VariablesArguments;
    type Response = crate::VariablesResponse;
}

/// Set the variable with the given name in the variable container to a new value. Clients should only call this request if the corresponding capability `supportsSetVariable` is true.
/// If a debug adapter implements both `setVariable` and `setExpression`, a client will only use `setExpression` if the variable has an `evaluateName` property.
pub enum SetVariable {}

impl Request for SetVariable {
    const COMMAND: &'static str = "setVariable";
    type Arguments = crate::SetVariableArguments;
    type Response = crate::SetVariableResponse;
}

/// The request retrieves the source code for a given source reference.
pub enum Source {}

impl Request for Source {
    const COMMAND: &'static str = "source";
    type Arguments = crate::SourceArguments;
    type Response = crate::SourceResponse;
}

/// The request retrieves a list of all threads.
pub enum Threads {}

impl Request for Threads {
    const COMMAND: &'static str = "threads";
    type Arguments = ();
    type Response = crate::ThreadsResponse;
}

/// The request terminates the threads with the given ids.
/// Clients should only call this request if the corresponding capability `supportsTerminateThreadsRequest` is true.
pub enum TerminateThreads {}

impl Request for TerminateThreads {
    const COMMAND: &'static str = "terminateThreads";
    type Arguments = crate::TerminateThreadsArguments;
    type Response = ();
}

/// Modules can be retrieved from the debug adapter with this request which can either return all modules or a range of modules to support paging.
/// Clients should only call this request if the corresponding capability `supportsModulesRequest` is true.
pub enum Modules {}

impl Request for Modules {
    const COMMAND: &'static str = "modules";
    type Arguments = crate::ModulesArguments;
    type Response = crate::ModulesResponse;
}

/// Retrieves the set of all sources currently loaded by the debugged process.
/// Clients should only call this request if the corresponding capability `supportsLoadedSourcesRequest` is true.
pub enum LoadedSources {}

impl Request for LoadedSources {
    const COMMAND: &'static str = "loadedSources";
    type Arguments = crate::LoadedSourcesArguments;
    type Response = crate::LoadedSourcesResponse;
}

/// Evaluates the given expression in the context of a stack frame.
/// The expression has access to any variables and arguments that are in scope.
pub enum Evaluate {}

impl Request for Evaluate {
    const COMMAND: &'static str = "evaluate";
    type Arguments = crate::EvaluateArguments;
    type Response = crate::EvaluateResponse;
}

/// Evaluates the given `value` expression and assigns it to the `expression` which must be a modifiable l-value.
/// The expressions have access to any variables and arguments that are in scope of the specified frame.
/// Clients should only call this request if the corresponding capability `supportsSetExpression` is true.
/// If a debug adapter implements both `setExpression` and `setVariable`, a client uses `setExpression` if the variable has an `evaluateName` property.
pub enum SetExpression {}

impl Request for SetExpression {
    const COMMAND: &'static str = "setExpression";
    type Arguments = crate::SetExpressionArguments;
    type Response = crate::SetExpressionResponse;
}

/// This request retrieves the possible step-in targets for the specified stack frame.
/// These targets can be used in the `stepIn` request.
/// Clients should only call this request if the corresponding capability `supportsStepInTargetsRequest` is true.
pub enum StepInTargets {}

impl Request for StepInTargets {
    const COMMAND: &'static str = "stepInTargets";
    type Arguments = crate::StepInTargetsArguments;
    type Response = crate::StepInTargetsResponse;
}

/// This request retrieves the possible goto targets for the specified source location.
/// These targets can be used in the `goto` request.
/// Clients should only call this request if the corresponding capability `supportsGotoTargetsRequest` is true.
pub enum GotoTargets {}

impl Request for GotoTargets {
    const COMMAND: &'static str = "gotoTargets";
    type Arguments = crate::GotoTargetsArguments;
    type Response = crate::GotoTargetsResponse;
}

/// Returns a list of possible completions for a given caret position and text.
/// Clients should only call this request if the corresponding capability `supportsCompletionsRequest` is true.
pub enum Completions {}

impl Request for Completions {
    const COMMAND: &'static str = "completions";
    type Arguments = crate::CompletionsArguments;
    type Response = crate::CompletionsResponse;
}

/// Retrieves the details of the exception that caused this event to be raised.
/// Clients should only call this request if the corresponding capability `supportsExceptionInfoRequest` is true.
pub enum ExceptionInfo {}

impl Request for ExceptionInfo {
    const COMMAND: &'static str = "exceptionInfo";
    type Arguments = crate::ExceptionInfoArguments;
    type Response = crate::ExceptionInfoResponse;
}

/// Reads bytes from memory at the provided location.
/// Clients should only call this request if the corresponding capability `supportsReadMemoryRequest` is true.
pub enum ReadMemory {}

impl Request for ReadMemory {
    const COMMAND: &'static str = "readMemory";
    type Arguments = crate::ReadMemoryArguments;
    type Response = crate::ReadMemoryResponse;
}

/// Writes bytes to memory at the provided location.
/// Clients should only call this request if the corresponding capability `supportsWriteMemoryRequest` is true.
pub enum WriteMemory {}

impl Request for WriteMemory {
    const COMMAND: &'static str = "writeMemory";
    type Arguments = crate::WriteMemoryArguments;
    type Response = crate::WriteMemoryResponse;
}

/// Disassembles code stored at the provided location.
/// Clients should only call this request if the corresponding capability `supportsDisassembleRequest` is true.
pub enum Disassemble {}

impl Request for Disassemble {
    const COMMAND: &'static str = "disassemble";
    type Arguments = crate::DisassembleArguments;
    type Response = crate::DisassembleResponse;
}

/// Looks up information about a location reference previously returned by the debug adapter.
pub enum Locations {}

impl Request for Locations {
    const COMMAND: &'static str = "locations";
    type Arguments = crate::LocationsArguments;
    type Response = crate::LocationsResponse;
}
