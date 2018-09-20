Performance
===========


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

Fibers Instead of Threads
*************************

System threads work well to distribute CPU-bound workloads, but are expensive when used in a 1-1 ratio with network requests.

Newer programming languages and paradigms have introduced the idea of an eventloop and/or green threading: lightweight threads that are multiplexed on a single thread. These lightweight threads can use operating system level constructs to handle networking, similar to classic threads. The benefits are:

* memory: threads implemented in the application do not require the full stack that system threads too, and thus can be smaller.
*
