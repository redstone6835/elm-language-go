// Package abi defines the Go view of the fixed ELM carrier lifecycle ABI.
package abi

const (
	// CarrierVersion is the first Go carrier ABI version.
	CarrierVersion uint32 = 1
	// LifecycleContextV1Size is the fixed byte size shared with Rust.
	LifecycleContextV1Size uint32 = 64
	// RuntimeV1Size is the fixed byte size of the unresolved entry table.
	RuntimeV1Size uint32 = 80
)

// Status is the signed result of a lifecycle hook. Zero reports success.
type Status int32

// LifecyclePhase identifies one runtime lifecycle callback.
type LifecyclePhase uint32

const (
	// PhaseInitialize creates the runtime, heap, scheduler, and metadata.
	PhaseInitialize LifecyclePhase = 1
	// PhasePause enters a resumable scheduler and GC stop boundary.
	PhasePause LifecyclePhase = 2
	// PhaseResume leaves the resumable stop boundary.
	PhaseResume LifecyclePhase = 3
	// PhaseQuiesce enters the terminal unload boundary.
	PhaseQuiesce LifecyclePhase = 4
	// PhaseFinalize terminates the runtime inside the unload boundary.
	PhaseFinalize LifecyclePhase = 5
)

// LifecycleContextV1 is the pointer-free context supplied to Go lifecycle
// entrypoints. Multi-byte fields use the target's little-endian ABI.
type LifecycleContextV1 struct {
	ABIVersion uint32
	StructSize uint32
	CellID     uint64
	Generation uint64
	Phase      LifecyclePhase
	Flags      uint32
	Reserved   [4]uint64
}

// RuntimeV1 is the Go view of the unresolved carrier entry table. Each entry
// is an address value, not a Go function pointer. The ELM loader validates
// executable ranges and relocations before the carrier call gate invokes an
// entry with the C signature:
//
//	int32 hook(const LifecycleContextV1 *context)
//
// A Go panic must be contained before this boundary.
type RuntimeV1 struct {
	ABIVersion uint32
	StructSize uint32
	Initialize uint64
	Pause      uint64
	Resume     uint64
	Quiesce    uint64
	Finalize   uint64
	Reserved   [4]uint64
}

// Valid reports whether every required unresolved entry is present and all V1
// reserved fields are zero. It does not prove that an address is executable.
func (runtime *RuntimeV1) Valid() bool {
	if runtime == nil ||
		runtime.ABIVersion != CarrierVersion ||
		runtime.StructSize != RuntimeV1Size ||
		runtime.Initialize == 0 ||
		runtime.Pause == 0 ||
		runtime.Resume == 0 ||
		runtime.Quiesce == 0 ||
		runtime.Finalize == 0 {
		return false
	}
	return runtime.Reserved[0] == 0 &&
		runtime.Reserved[1] == 0 &&
		runtime.Reserved[2] == 0 &&
		runtime.Reserved[3] == 0
}

// Valid reports whether the fixed header, phase, and V1 reserved fields are
// acceptable. Integrated modules may use zero cell and generation values.
func (context *LifecycleContextV1) Valid() bool {
	if context == nil ||
		context.ABIVersion != CarrierVersion ||
		context.StructSize != LifecycleContextV1Size ||
		context.Flags != 0 {
		return false
	}
	if context.Phase < PhaseInitialize || context.Phase > PhaseFinalize {
		return false
	}
	return context.Reserved[0] == 0 &&
		context.Reserved[1] == 0 &&
		context.Reserved[2] == 0 &&
		context.Reserved[3] == 0
}
