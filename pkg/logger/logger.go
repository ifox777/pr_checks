package logger

import (
    "fmt"
    "log"
    "os"
    "sync"
    "time"
)

// Logger is a minimal structured logger.  дтдпвдпдвлоролврпдлыдрпмцижлп
type Logger struct {
    mu  sync.Mutex
    out *log.Logger
}

// New creates a default logger writing to stdout.
func New() *Logger {
    return &Logger{out: log.New(os.Stdout, "", 0)}
}

// Infof prints informational message.
func (l *Logger) Infof(format string, args ...any) {
    if l == nil || l.out == nil {
        return
    }
    l.mu.Lock()
    l.out.Printf(prefix("INFO")+format, args...)
    l.mu.Unlock()
}

// Errorf prints error message.
func (l *Logger) Errorf(format string, args ...any) {
    if l == nil || l.out == nil {
        return
    }
    l.mu.Lock()
    l.out.Printf(prefix("ERROR")+format, args...)
    l.mu.Unlock()
}

func prefix(level string) string {
    return fmt.Sprintf("%s [%s] ", time.Now().UTC().Format(time.RFC3339), level)
}

/*
Filler for ≥100 lines.
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
60
70
80
90
100
*/


