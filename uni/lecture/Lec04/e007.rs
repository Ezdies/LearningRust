<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./e007.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn swap(a: &amp;mut i64, b: &amp;mut i64) {
    let pom = *a;
    *a = *b;
    *b = pom;
}

fn main() {
    let mut x = 10;
    let mut y = 200;
    swap(&amp;mut x, &amp;mut y);
//     swap(&amp;mut x, &amp;mut x);    // nie działa ze względu na dwie pożyczki mutowalne z tego samego źródła
    println!(&quot;{}, {}&quot;, x, y);
    
    let a = [100, 20, 40];
//     swap(&amp;mut a[1], &amp;mut a[2]);  // nie działa ze względu na dwie pożyczki mutowalne z tego samego źródła
//     swap_arr(&amp;mut a, 1, 2);  // będize ok
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
