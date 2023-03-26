<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./d044.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let s0 = &quot;Witaj, świecie!&quot;;
    let mut s2 = String::new();
    
    for c in s0.chars().rev() {
        s2.push(c);
    }
    println!(&quot;{}&quot;, s2);
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
