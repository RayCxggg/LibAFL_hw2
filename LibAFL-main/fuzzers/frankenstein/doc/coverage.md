# Frankenstein code coverage分析

Frankenstein设计了一套WebUI来供测试人员使用，但无论是在web、代码或是终端输出中，我们都没有发现论文中提到的基于code-coverage的反馈，因此无法确定Frankenstein是如何对测试样例进行变化的。在整个代码库中，我们只在前端`uc.py`脚本中找到了与代码覆盖率相关的部分

```python
def get_tracefile(self):
    trace = ""
    for address in self.coverage_pc:
        trace += "0x%x\n" % address
    return trace.encode()
```        