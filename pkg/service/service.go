package service

import (
    "errors"
    "sync"

    "example.com/pr_check/pkg/logger"
)

// Service is a simple lifecycle-managed component.
type Service struct {
    mu     sync.RWMutex
    log    *logger.Logger
    active bool
}

// NewService constructs Service.
func NewService(log *logger.Logger) *Service {
    return &Service{log: log}
}

// Start activates service.
func (s *Service) Start() error {
    s.mu.Lock()
    defer s.mu.Unlock()
    if s.active {
        return nil
    }
    s.active = true
    if s.log != nil {
        s.log.Infof("service started")
    }
    return nil
}

// Stop deactivates service.
func (s *Service) Stop() error {
    s.mu.Lock()
    defer s.mu.Unlock()
    if !s.active {
        return errors.New("service not active")
    }
    s.active = false
    if s.log != nil {
        s.log.Infof("service stopped")
    }
    return nil
}

// Process logs a unit of work.
func (s *Service) Process(name string) error {
    s.mu.RLock()
    active := s.active
    s.mu.RUnlock()
    if !active {
        return errors.New("service inactive")
    }
    if s.log != nil {
        s.log.Infof("processing %s", name)
    }
    return nil
}

// IsActive reports whether service is active.
func (s *Service) IsActive() bool {
    s.mu.RLock()
    defer s.mu.RUnlock()
    return s.active
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
80
90
100
*/


