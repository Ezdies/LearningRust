<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./d005.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn wyswietl(t: &amp;[i32]) {
    println!(&quot;{:?}&quot;, t);
}

fn main() {
    let mut tablica = [10, 20, 40, 100];
    wyswietl(&amp;tablica);
    println!(&quot;{}&quot;, tablica[1]);
    tablica[2] = 567;
//     tablica[100] = 987;  // błąd -- na poziomie kompilacji
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
