use neon::prelude::*;
use rayon::prelude::*;

fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn calculate_factorial(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Extract the argument from JavaScript
    let num = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;

    // Calculate the factorial
    let result = factorial(num);

    // Create a JavaScript number and return it
    Ok(cx.number(result as f64))
}


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}
fn calculate_sum_of_squares(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_array = cx.argument::<JsArray>(0)?;

    // Convert JsArray to Vec<f64>
    let js_values: Vec<f64> = js_array
        .to_vec(&mut cx)?
        .into_iter()
        .map(|item| item.downcast::<JsNumber, _>(&mut cx).unwrap().value(&mut cx))
        .collect();

    // Use parallel iterator for better performance
    let sum_of_squares: f64 = js_values
        .par_iter()
        .map(|x| x * x)
        .sum();

    Ok(cx.number(sum_of_squares))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("calculateFactorial", calculate_factorial)?;
    cx.export_function("calculateSumOfSquare", calculate_sum_of_squares)?;
    Ok(())
}
