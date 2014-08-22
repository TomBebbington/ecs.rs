nexu
====
Nexu is a programming language and game engine that JIT compiles an ECS

```
comp Position:
	float x
	float y

comp Velocity:
	float x
	float y

comp Mass:
	float mass

sys Physics for Position + Mass:
	fn update with float delta:
		[Velocity].y += [Mass].mass * delta
		if [Velocity].y <= 1:
			[Velocity].y = 0
```