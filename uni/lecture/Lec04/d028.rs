<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>./d028.rs</title>
</head>
<body bgcolor="#ffffff" text="#000000">
<pre>
fn main() {
    let s0 = &quot;Witaj, świecie!&quot;;
    let mut s1 = &quot;Ala ma kota&quot;.to_string();
    let s2 = String::new();
    let s3 = String::from(&quot;Pies i kot.&quot;);
    
    s1.push_str(&quot; i psa&quot;);
    s1.push('.');
    println!(&quot;{}&quot;, s1);
    
    println!(&quot;{:?}&quot;, s0.get(..3));
    println!(&quot;{:?}&quot;, s0.get(3..));
    println!(&quot;{:?}&quot;, s0.get(1..8));
    println!(&quot;{:?}&quot;, s0.get(180..));

    println!(&quot;{:?}&quot;, s0.chars().nth(6));
    println!(&quot;{:?}&quot;, s0.bytes().nth(6));
    println!(&quot;{:?}&quot;, s0.chars().nth(7));
    println!(&quot;{:?}&quot;, s0.bytes().nth(7));
    println!(&quot;{:?}&quot;, s0.chars().nth(8));
    println!(&quot;{:?}&quot;, s0.bytes().nth(8));
    println!(&quot;{:?}&quot;, s0.chars().nth(9));
    println!(&quot;{:?}&quot;, s0.bytes().nth(9));
    println!(&quot;{:?}&quot;, s0.chars().nth(99));
    println!(&quot;{:?}&quot;, s0.bytes().nth(99));

    println!(&quot;{:?}&quot;, s0.len());
    println!(&quot;{:?}&quot;, s0.bytes().len());
//     println!(&quot;{:?}&quot;, s0.chars().len());  // nie ma
}

</pre>
<hr>
syntax highlighted by <a href="http://www.palfrader.org/code2html">Code2HTML</a>, v. 0.9.1
</body>
</html>
