package abi

import (
	"testing"
	"unsafe"
)

func TestLifecycleContextV1Layout(t *testing.T) {
	var context LifecycleContextV1
	if got := unsafe.Sizeof(context); got != uintptr(LifecycleContextV1Size) {
		t.Fatalf("context size = %d, want %d", got, LifecycleContextV1Size)
	}
	if got := unsafe.Offsetof(context.CellID); got != 8 {
		t.Fatalf("CellID offset = %d, want 8", got)
	}
	if got := unsafe.Offsetof(context.Generation); got != 16 {
		t.Fatalf("Generation offset = %d, want 16", got)
	}
	if got := unsafe.Offsetof(context.Phase); got != 24 {
		t.Fatalf("Phase offset = %d, want 24", got)
	}
	if got := unsafe.Offsetof(context.Reserved); got != 32 {
		t.Fatalf("Reserved offset = %d, want 32", got)
	}
}

func TestLifecycleContextV1Validation(t *testing.T) {
	context := LifecycleContextV1{
		ABIVersion: CarrierVersion,
		StructSize: LifecycleContextV1Size,
		CellID:     7,
		Generation: 3,
		Phase:      PhaseQuiesce,
	}
	if !context.Valid() {
		t.Fatal("valid context was rejected")
	}
	context.Reserved[2] = 1
	if context.Valid() {
		t.Fatal("reserved data must be rejected")
	}
}

func TestRuntimeV1LayoutAndValidation(t *testing.T) {
	var runtime RuntimeV1
	if got := unsafe.Sizeof(runtime); got != uintptr(RuntimeV1Size) {
		t.Fatalf("runtime size = %d, want %d", got, RuntimeV1Size)
	}
	if got := unsafe.Offsetof(runtime.Initialize); got != 8 {
		t.Fatalf("Initialize offset = %d, want 8", got)
	}
	if got := unsafe.Offsetof(runtime.Pause); got != 16 {
		t.Fatalf("Pause offset = %d, want 16", got)
	}
	if got := unsafe.Offsetof(runtime.Resume); got != 24 {
		t.Fatalf("Resume offset = %d, want 24", got)
	}
	if got := unsafe.Offsetof(runtime.Quiesce); got != 32 {
		t.Fatalf("Quiesce offset = %d, want 32", got)
	}
	if got := unsafe.Offsetof(runtime.Finalize); got != 40 {
		t.Fatalf("Finalize offset = %d, want 40", got)
	}
	if got := unsafe.Offsetof(runtime.Reserved); got != 48 {
		t.Fatalf("Reserved offset = %d, want 48", got)
	}

	runtime = RuntimeV1{
		ABIVersion: CarrierVersion,
		StructSize: RuntimeV1Size,
		Initialize: 0x1000,
		Pause:      0x1010,
		Resume:     0x1020,
		Quiesce:    0x1030,
		Finalize:   0x1040,
	}
	if !runtime.Valid() {
		t.Fatal("valid runtime table was rejected")
	}
	runtime.Pause = 0
	if runtime.Valid() {
		t.Fatal("missing pause entry must be rejected")
	}
}
