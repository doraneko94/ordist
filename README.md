# ordist
Order distance package written in Rust, which can be used from Python.

## usage
```
git clone https://github.com/doraneko94/ordist
pip install ./ordistpy
python
>>> import ordist as odt
>>> a = ["A", "B", "C"]
>>> b = ["A", "C", "B"]
>>> odt.spearman_dist(a, b)
2
```