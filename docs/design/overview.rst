Major Design Considerations
===========================

Disp has a couple guiding philosophies that drive it's roadmap and development.

Developer Productivity
****************

One is developer productivity, ensuring logic is as intended. Developers should be able to efficiently write code and tackle common challenges around coding, including:

* writing new code
* understanding legacy code
* resolving bugs with existing code

Performance
***********

With developer productivity comes a common problem that affects successful software programs: performance. It's often said that performance is often not a primary driver of language choice, and premature optimization is the root of all evil.

However, the optimization process can be significantly more expensive or cheap depending on how the language itself is designed. Examples of language that are difficult to optimize include Python, which require developers to author non-python code to resolve (such as Cython for a Python subset, or C code).

Hard-To-Optimize Language (Python):

* requires coding outside the language to optimize

Easy-To-Optimize Language (Java):

* can get really good performance (1-2x C) before needing to move out of the language
* sticking to the same language enables:
  * easier developer contribution
  * simpler build tooling

Compare this to languages which often do not require significant optimization outside the language, such as Java: the VM itself is efficient enough for a wide range of purposes, and thus enables much better developer contribution
