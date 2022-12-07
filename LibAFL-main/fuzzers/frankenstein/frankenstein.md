# Frankenstein: Advanced Wireless Fuzzing to Exploit New Bluetooth Escalation Targets

今天为大家推荐来自Secure Mobile Networking Lab, TU Darmstadt的针对蓝牙协议的模糊测试工作：Frankenstein: Advanced Wireless Fuzzing to Exploit New Bluetooth Escalation Targets，
该工作收录于29th USENIX Security Symposium.

长久以来，无线通讯协议的设计和实现安全问题一直困扰着开发者。由于大多数固件实现是闭源的，所以模糊测试（fuzzing）仍然是寻找部署的系统中的远程代码执行（RCE）漏洞的主要方法。现有的over-the-air fuzzing手段有显著的缺点，比如测试速率、可重复性以及debug能力方面的限制。

本文中，作者提出了Frankenstein——一种基于固件模拟的fuzzing框架，有效解决了上述模糊测试技术的缺点。Frankenstein选择利用固件转储（firmware dump)，并为虚拟的芯片调制解调器（modem）提供测试输入。作者设计的高速fuzzing方法能够与目标操作系统保持互通性，进而触发真实的协议栈行为。

在实验中，作者利用Frankenstein发现了Broadcom和Cypress蓝牙协议栈的3个zero-click漏洞，这些蓝牙芯片被广泛应用在Apple、Samsung和Rasberry Pi等设备中。利用蓝牙芯片的RCE漏洞，攻击者可以将权限提升到突破单个芯片的边界。作者发现了一个由WiFi和蓝牙协议栈共存而导致的漏洞，该漏洞可以引发多种操作系统的内核崩溃。文章还发现了蓝牙5.2协议的设计缺陷，该缺陷可以导致host的link key被非法提取。因为关闭蓝牙并不能完全停止芯片的运行，所以RCE漏洞是难以防御的。磁瓦， 通过对其他设备的测试，作者还发现了一个与芯片类型无关的Andriod RCE漏洞BlueFrag，
