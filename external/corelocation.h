enum STATUS {
  OK,
  NOT_ENABLED,
  NOT_RETURNED,
  STALE,
};

typedef struct LocInfo {
  double latitude;
  double longitude;
  double h_accuracy;
  double altitude;
  double v_accuracy;
  enum STATUS status;
  int error_duration;
} LocInfo;


LocInfo run(void);
