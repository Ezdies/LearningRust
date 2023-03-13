<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b038.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let mut a = 10;
    let b = 20;

    let x = loop {
        if a &gt;= b {
            break a;
        }
        a += 1;
        println!(&quot;{}&quot;, a);
    };

    println!(&quot;{:?}&quot;, x);
}


</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
