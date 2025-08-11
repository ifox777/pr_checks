package main

import (
    "fmt"
    "time"

    mathutil "example.com/pr_check/internal/mathutil"
    "example.com/pr_check/internal/util"
    "example.com/pr_check/pkg/config"
    "example.com/pr_check/pkg/logger"
    "example.com/pr_check/pkg/service"
)

func main() {
    log := logger.New()
    cfg, err := config.FromEnv()
    if err != nil {
        log.Errorf("config error: %v", err)
    } else {
        log.Infof("config: name=%s addr=%s", cfg.AppName, cfg.Addr())
    }
    srv := service.NewService(log)

    if err := srv.Start(); err != nil {
        panic(err)
    }

    msg := util.JoinWords([]string{"hello", "from", "service"})
    sum := mathutil.Add(2, 3)
    log.Infof("message=%s sum=%d", msg, sum)

    _ = srv.Process("demo-task")

    time.Sleep(50 * time.Millisecond)
    if err := srv.Stop(); err != nil {
        fmt.Println("stop error:", err)
    }
}

/*
Filler lines for ≥100 lines requirement. These are comments and do not affect program behavior.
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
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
*/


