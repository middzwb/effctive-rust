fn main() {
  println!("hello, world");

  // warning: error code!
  let mut v = vec![1, 2, 3, 4, 5];

  let first = &v[0];

  v.push(6);

  println!("The first element is: {}", first);

  // 类似的 c++的vec也是在内存中线性布局，当容量不足时，也会重新申请，导致之前的引用失效。
  // 如下的样例c++ code演示了这一问题
  /**
    c++ vector reference
  {
    vector<int> v;
    v.resize(10);
    v[0] = 100;
    auto& r = v[0];
    cout << &v[0] << " " << &r << endl;
    v.resize(100000);
    cout << &v[0] << " " << &r << endl;
    cout << v[0] << " " << r << endl;
  }
  **/
}
