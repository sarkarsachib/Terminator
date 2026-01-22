package motion

type SensorFrame struct {
	Topic   string
	Payload []byte
}

type SensorManager struct{}

func (m *SensorManager) Poll() (*SensorFrame, error) {
	return &SensorFrame{Topic: "motion.stub", Payload: []byte("noop")}, nil
}
