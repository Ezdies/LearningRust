<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./c3007.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn powitaj(imie: &amp;str) {
    println!(&quot;Witaj, {}!&quot;, imie);
}

fn powitaj_z_przejeciem(imie: String) {
    println!(&quot;Witaj, {}!&quot;, imie);
}

fn main() {
    let imie = &quot;Edek&quot;;
    powitaj(imie);
    let inne_imie: String = &quot;Felek&quot;.to_string();
    powitaj(&amp;inne_imie);
    powitaj(&amp;inne_imie);
    powitaj_z_przejeciem(inne_imie);
//     powitaj_z_przejeciem(inne_imie); // tu już nie działa
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
