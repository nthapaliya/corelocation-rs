enum STATUS {
  OK,
  NOT_ENABLED,
  NOT_RETURNED,
  STALE,
};

typedef struct LocInfo {
  double latitude;
  double longitude;
  int h_accuracy;
  int altitude;
  int v_accuracy;
  enum STATUS status;
  int error_duration;
} LocInfo;


LocInfo run(void);
