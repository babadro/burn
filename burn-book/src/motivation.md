# Why Burn?

Why create an entirely new deep learning framework from scratch when 
PyTorch, TensorFlow, and other frameworks already exist? Spoiler alert: Burn isn't merely a 
replication of PyTorch or TensorFlow in Rust. It is a novel approach, 
making the right compromises in the right areas to enable exceptional flexibility, high performance, 
and a seamless developer experience. Burn isn’t a framework specialized for a single application.
It is designed to be suitable for a wide range of research and production uses. 
The foundation of Burn's design revolves around three key user profiles.

**Machine Learning Researchers** require tools to construct and execute experiments efficiently.
It’s essential for them to iterate quickly on their ideas and design testable experiments 
to allow for novel discoveries. The framework should facilitate the swift implementation of 
cutting-edge algorithms and ensure fast execution for testing.

**Machine Learning Engineers** focus on robustness, seamless deployment, and cost-effective operations. 
They seek dependable models to achieve their objectives without excessive expense. 
From training to inference, the whole machine learning workflow must be efficient and predictable.

**Low-level Software Engineers** working with hardware vendors need their processing units to run 
models as fast as possible to gain an edge on competitors. This endeavor involves harnessing
hardware-specific features such as Tensor Core for Nvidia. Since they are mostly working at system
level, they need absolute control over how the computation is executed.

The goal of Burn is to satisfy all of those personas!
