<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b019.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let mut a = 10;
    let b = 20;

    while a &lt; b {
        a += 1;
        if a == 15 {
            continue;
        }
        if a == 17 {
            break;
        }
        println!(&quot;{}&quot;, a);
    }
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
