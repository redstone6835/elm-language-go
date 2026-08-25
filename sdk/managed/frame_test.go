package managed

import (
	"testing"
	"unsafe"
)

func TestFrameLayout(t *testing.T) {
	var frame Frame
	if got := unsafe.Sizeof(frame); got != 256 {
		t.Fatalf("Frame size = %d, want 256", got)
	}
	if got := unsafe.Offsetof(frame.CellID); got != 16 {
		t.Fatalf("CellID offset = %d, want 16", got)
	}
	if got := unsafe.Offsetof(frame.Payload); got != 48 {
		t.Fatalf("Payload offset = %d, want 48", got)
	}
	if got := unsafe.Offsetof(frame.Reserved); got != 240 {
		t.Fatalf("Reserved offset = %d, want 240", got)
	}
}

func TestFrameValidation(t *testing.T) {
	frame := Frame{
		ABIVersion: ABIVersion,
		StructSize: 256,
		Operation:  1,
		CellID:     7,
		Generation: 2,
		RequestID:  3,
	}
	if !frame.Valid() {
		t.Fatal("valid frame was rejected")
	}
	frame.Reserved[0] = 1
	if frame.Valid() {
		t.Fatal("reserved data must be rejected")
	}
}
