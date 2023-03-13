<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b034.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn nwd(a : i32, b : i32) -&gt; i32 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if b &gt; a {
        nwd(b%a, a)
    } else {
        nwd(a%b, b)
    }
}

fn main() {
    println!(&quot;{}&quot;, nwd(100, 30));
}


</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
