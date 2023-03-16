<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>w03/c0016.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    println!(&quot;{}&quot;, i32::MAX);
    println!(&quot;{}&quot;, i64::MIN);
    println!(&quot;{}&quot;, u8::MAX);
    println!(&quot;{}&quot;, u32::MIN);

    println!(&quot;{}&quot;, f32::MAX);
    println!(&quot;{}&quot;, f32::MIN);
    println!(&quot;{}&quot;, f64::MAX);
    println!(&quot;{}&quot;, f64::MIN);
    println!(&quot;{}&quot;, f32::MIN_POSITIVE);
    println!(&quot;{}&quot;, f32::NAN);
    println!(&quot;{}&quot;, f32::INFINITY);
    
    println!(&quot;{}&quot;, usize::MIN);
    println!(&quot;{}&quot;, usize::MAX);
    println!(&quot;{}&quot;, isize::MIN);
    println!(&quot;{}&quot;, isize::MAX);

    println!(&quot;{}&quot;, usize::MAX as u128 == u64::MAX as u128);
    println!(&quot;{}&quot;, usize::MAX as u128 == u32::MAX as u128);
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
