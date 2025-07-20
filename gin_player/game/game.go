package game

import (
	"fmt"
	"log"
	"net/http"
	"sync"
	"sync/atomic"
	"time"
	"encoding/json"
)

func (a *AttackType) UnmarshalJSON(data []byte) error {
	var raw map[string]uint64
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	for k, v := range raw {
		a.Type = k
		a.Value = v
		return nil
	}
	a.Type = ""
	a.Value = 0
	return nil
}


type AttackType struct {
	Type  string `json:"type"`
	Value uint64 `json:"value,omitempty"` // Используется для Burn, FibRec и FibIter
}

// GameSettings holds configuration for the DDoS game
type GameSettings struct {
	ConnectionsAmount int        `json:"connections_amount"`
	DelayMs           int        `json:"delay_ms"`
	RoundSec          int        `json:"round_sec"`
	AttackType        AttackType `json:"attack_type"`
	Targets           []string   `json:"targets"`
}

// start_attack_game launches the attack game logic
func Start_attack_game(settings GameSettings) error {
	var wg sync.WaitGroup
	var isPlaying atomic.Bool
	isPlaying.Store(true)
	client := &http.Client{}

	// Global stopper
	wg.Add(1)
	go func() {
		defer wg.Done()
		time.Sleep(time.Duration(settings.RoundSec) * time.Second)
		isPlaying.Store(false)
		log.Println("Round finished")
	}()

	for _, target := range settings.Targets {
		wg.Add(1)
		go func(target string) {
			defer wg.Done()
			attack_target(&isPlaying, settings, client, target)
		}(target)
	}

	wg.Wait()
	return nil
}

// attack_target runs connections_amount concurrent attacks against a target
func attack_target(isPlaying *atomic.Bool, settings GameSettings, client *http.Client, target string) {
	var wg sync.WaitGroup

	for idx := 0; idx < settings.ConnectionsAmount; idx++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			log.Printf("Attacker #%d → %s started", idx, target)
			for isPlaying.Load() {

				var path string
				switch settings.AttackType.Type {
				case "burn", "fib_iter", "fib_rec":
					path = fmt.Sprintf("%s/%d", settings.AttackType.Type, settings.AttackType.Value)
				case "bomb":
					path = settings.AttackType.Type
				default:
					log.Printf("Unknown attack type: %s", settings.AttackType.Type)
					return
				}

				url := fmt.Sprintf("http://%s/%s", target, path)

				resp, err := client.Post(url, "application/json", nil)
				if err != nil {
					log.Printf("Request to %s failed: %v", target, err)
					return
				}
				log.Printf("Request to %s status: %d", target, resp.StatusCode)
				resp.Body.Close()
				time.Sleep(time.Duration(settings.DelayMs) * time.Millisecond)
			}
			log.Printf("Attacker #%d → %s stopped", idx, target)
		}(idx)
	}

	wg.Wait()
	log.Printf("Attack on target %s is finished", target)
}

// fibonacci_iterative returns the nth Fibonacci number iteratively
func Fibonacci_iterative(n uint64) uint64 {
	switch n {
	case 0:
		return 0
	case 1:
		return 1
	}
	var prev, curr uint64 = 0, 1
	for i := uint64(2); i <= n; i++ {
		next := prev + curr
		prev = curr
		curr = next
	}
	return curr
}
