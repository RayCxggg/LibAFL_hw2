# Frankenstein: Advanced Wireless Fuzzing to Exploit New Bluetooth Escalation Targets

今天为大家推荐来自Secure Mobile Networking Lab, TU Darmstadt的针对蓝牙协议的模糊测试工作：Frankenstein: Advanced Wireless Fuzzing to Exploit New Bluetooth Escalation Targets，
该工作收录于29th USENIX Security Symposium.

长久以来，无线通讯协议的设计和实现安全问题一直困扰着开发者。由于大多数固件实现是闭源的，所以模糊测试（fuzzing）仍然是寻找部署的系统中的远程代码执行（RCE）漏洞的主要方法。现有的over-the-air fuzzing手段有显著的缺点，比如测试速率、可重复性以及debug能力方面的限制。

本文中，作者提出了Frankenstein——一种基于固件仿真的fuzzing框架，有效解决了上述模糊测试技术的缺点。Frankenstein选择利用固件转储（firmware dump)，并为虚拟的芯片调制解调器（modem）提供测试输入。作者设计的高速fuzzing方法能够与目标操作系统保持互通性，进而触发真实的协议栈行为。

在实验中，作者利用Frankenstein发现了Broadcom和Cypress蓝牙协议栈的3个zero-click漏洞，这些蓝牙芯片被广泛应用在Apple、Samsung和Rasberry Pi等设备中。利用蓝牙芯片的RCE漏洞，攻击者可以将权限提升到突破单个芯片的边界。作者发现了一个由WiFi和蓝牙协议栈共存而导致的漏洞，该漏洞可以引发多种操作系统的内核崩溃。文章还发现了蓝牙5.2协议的设计缺陷，该缺陷可以导致host的link key被非法提取。因为关闭蓝牙并不能完全停止芯片的运行，所以RCE漏洞是难以防御的。磁瓦， 通过对其他设备的测试，作者还发现了一个与芯片类型无关的Andriod RCE漏洞BlueFrag。


## 核心贡献

对于Broadcom芯片，作者通过对配对阶段之前能够获得的固件进行系统化fuzzing，找到了对应的蓝牙zero-click RCE漏洞。而Cypress芯片于2016年获取了Broadcom的实现代码，虽然此后二者独立开发，但是针对它们的测试方法是相似的。仿真和fuzzing提供了对没有文档记录的固件的深入分析。作者设计了一套C语言环境来与固件镜像交互，测试假想示例并缩减相关的代码路径。

总体来说，本文的突出贡献主要有如下四点：
1. 设计和实现了仿真框架Frankenstein来运行大部分固件，包括注入无线数据帧和与host交互；
2. 发现3个zero-click芯片漏洞，两个针对经典蓝牙，一个针对BLE；
3. 发现Android系统的BlueFrag漏洞；
4. 攻击WiFi和Bluetooth芯片的共存机制，导致设备需要完全重启，或者引发内核崩溃；
5. 揭露蓝牙5.2协议的一个设计缺陷，允许攻击者通过关闭的连接获取host link key；
6. 证明了用户无法通过关闭蓝牙来进行防御，因为芯片的重启机制没有被正确配置。
