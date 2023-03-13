<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./b032.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let mut a = 10;

    a = if a % 2 == 0 {
        a / 2
    } else {
        let pom = 3*a;
        pom + 1
    };

    println!(&quot;{:?}&quot;, a);
}


</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
