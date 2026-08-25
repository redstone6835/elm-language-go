package managed

const (
	// ABIVersion is the first managed frame ABI.
	ABIVersion uint32 = 1
	// PayloadLen is the fixed inline payload capacity.
	PayloadLen = 192
)

// Frame is the Go representation of the optional 256-byte managed carrier
// frame. Multi-byte values use the target's little-endian ABI.
type Frame struct {
	ABIVersion uint32
	StructSize uint32
	Operation  uint32
	Flags      uint32
	CellID     uint64
	Generation uint64
	RequestID  uint64
	PayloadLen uint32
	Reserved0  uint32
	Payload    [PayloadLen]byte
	Reserved   [2]uint64
}

// Valid reports whether fixed V1 fields can be sent to the managed plane.
func (frame *Frame) Valid() bool {
	if frame == nil {
		return false
	}
	return frame.ABIVersion == ABIVersion &&
		frame.StructSize == 256 &&
		frame.Operation != 0 &&
		frame.Flags == 0 &&
		frame.CellID != 0 &&
		frame.Generation != 0 &&
		frame.RequestID != 0 &&
		frame.PayloadLen <= PayloadLen &&
		frame.Reserved0 == 0 &&
		frame.Reserved[0] == 0 &&
		frame.Reserved[1] == 0
}
