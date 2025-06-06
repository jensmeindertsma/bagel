use super::interpreter::InterpreterError;

pub trait Reporter {
    fn report(error: &InterpreterError) {
        todo!()
    }
}

pub struct BasicPrinter {}

impl Reporter for BasicPrinter {}

pub struct FancyPrinter {}

impl Reporter for FancyPrinter {
    fn report(error: &InterpreterError) {
        // TODO: figure out how to print multiple errors like scanner here
    }
}
