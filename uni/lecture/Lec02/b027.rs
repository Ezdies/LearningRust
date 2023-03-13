<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b027.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let mut a = 10;

    let b = if a % 2 == 0 {
        a / 2
    } else {
        3 * a + 1
    };

    println!(&quot;{:?}&quot;, b);
}


</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
