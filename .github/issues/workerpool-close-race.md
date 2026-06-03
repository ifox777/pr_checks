# [Bug] WorkerPool panics when Submit races with Close

Labels: bug

## Описание

`WorkerPool.Submit` writes directly to the internal task channel, while `WorkerPool.Close` cancels the context and closes that same channel. If one goroutine submits work while another closes the pool, the submitter can panic with `send on closed channel` instead of receiving a safe no-op or an explicit error.

## Ожидаемый результат

Concurrent calls to `Submit` and `Close` should not panic. The pool should either reject new tasks after shutdown with an error-returning API or safely ignore submissions once the pool is closed.

## Шаги воспроизведения

1. Create a `WorkerPool` with one or more workers.
2. Start one goroutine that repeatedly calls `Submit`.
3. Start another goroutine that calls `Close` while submissions are still happening.
4. Observe that a submitter can panic when it sends to the closed channel.
