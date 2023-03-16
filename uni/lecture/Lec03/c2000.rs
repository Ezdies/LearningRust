<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>c2000.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn f1(x:f64) -&gt; f64 {
    x*x-2.0*x+7.0
}

fn f2(x:f64) -&gt; f64 {
    x-27.0
}

fn tablicuj(f:fn(f64)-&gt;f64, mut a:f64, b:f64, h:f64) {
    println!(&quot;==================================&quot;);
    while a &lt;=b {
        println!(&quot;f({}) = {}&quot;, a, f(a));
        a += h;
    }
    println!(&quot;==================================&quot;);
}

fn main() {
    tablicuj(f1, -0.1, 0.1, 0.01);
    tablicuj(f2, 0.0, 10.0, 2.5);
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
