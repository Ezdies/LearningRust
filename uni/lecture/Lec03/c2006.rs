<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>w03/c2006.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn powiekszona_o_1(x: i32) -&gt; i32 {
    x + 1
}

fn powieksz_o_1(x: &amp;mut i32) {
    *x += 1;
}

fn main() {
    let mut a = 12;
    
    let b = powiekszona_o_1(a);
    println!(&quot;{}&quot;, b == 13);
    
    powieksz_o_1(&amp;mut a);
    println!(&quot;{}&quot;, a == 13);
    powieksz_o_1(&amp;mut a);
    println!(&quot;{}&quot;, a == 14);
}

/*

przekazywanie danych do funkcji przez:
* wartość (czasem kopiowanie [typy kopiowalne, zwykle to typy proste], a czasem przeniesienie [inne])
* referencję (pożyczkę)
* referencję (pożyczkę) mutowalną

rozróżnienie pojęć: kopiowanie a klonowanie

*/

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
