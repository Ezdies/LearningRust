<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./d037.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let mut s1 = &quot;Ala ma kota&quot;.to_string();
    
    s1.push_str(&quot; i psa&quot;);
    s1.push('.');
    println!(&quot;{}&quot;, s1);
    
    println!(&quot;{:?}&quot;, s1.find('a'));
    println!(&quot;{:?}&quot;, s1.find('x'));
    println!(&quot;{:?}&quot;, s1.find(&quot;a&quot;));
    println!(&quot;{:?}&quot;, s1.find(&quot;kot&quot;));
    
    let s4 = s1.replace(&quot;kota&quot;, &quot;szczura&quot;);
    println!(&quot;{}&quot;, s4);
    
    let a = s1.as_bytes();
    println!(&quot;{:?}&quot;, a);
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
