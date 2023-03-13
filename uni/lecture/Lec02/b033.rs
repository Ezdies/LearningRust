<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b033.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn nwd(a : i32, b : i32) -&gt; i32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if b &gt; a {
        return nwd(b%a, a);
    }
    return nwd(a%b, b);
}

fn main() {
    println!(&quot;{}&quot;, nwd(100, 30));
}


</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
