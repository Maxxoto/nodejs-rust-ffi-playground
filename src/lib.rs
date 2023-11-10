use neon::prelude::*;
use neon::types::buffer::TypedArray;
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
     // Receive a JsTypedArray as an argument
     let typed_array: Handle<JsTypedArray<u32>> = cx.argument::<JsTypedArray<u32>>(0)?;

     // Access the underlying data as a slice of u32
    let data: &[u32] = typed_array.as_slice(&cx);

    // Convert the slice to a Vec<f64>
    let f64_data: Vec<f64> = data.iter().map(|&x| x as f64).collect();


     // Use parallel iterator for better performance
     let sum_of_squares: f64 = f64_data.par_iter().map(|x| x * x).sum();


     Ok(cx.number(sum_of_squares))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("calculateFactorial", calculate_factorial)?;
    cx.export_function("calculateSumOfSquare", calculate_sum_of_squares)?;
    Ok(())
}
