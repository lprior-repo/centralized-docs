package domain

import "testing"

func TestVersion(t *testing.T) {
	tests := []struct {
		name string
		want string
	}{
		{
			name: "version is set correctly",
			want: "0.1.0",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if Version != tt.want {
				t.Errorf("Version = %q, want %q", Version, tt.want)
			}
		})
	}
}
