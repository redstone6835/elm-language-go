package kernel

import (
	"testing"
	"unsafe"
)

func TestFixedLayouts(t *testing.T) {
	if got := unsafe.Sizeof(Owner{}); got != 16 {
		t.Fatalf("Owner size = %d, want 16", got)
	}
	if got := unsafe.Sizeof(Handle{}); got != 24 {
		t.Fatalf("Handle size = %d, want 24", got)
	}
}

func TestZeroIdentitiesAreInvalid(t *testing.T) {
	if (Owner{}).Valid() {
		t.Fatal("zero owner must be invalid")
	}
	if (Handle{}).Valid() {
		t.Fatal("zero handle must be invalid")
	}
}
