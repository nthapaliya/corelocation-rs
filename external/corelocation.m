#include "corelocation.h"

@implementation LocationDelegate: NSObject
- (void)locationManager:(CLLocationManager *)manager
     didUpdateLocations:(NSArray<CLLocation *> *)locations
{
  // To suppress annoying "unused variable" warning;
  (void)manager;
  (void)locations;

  CFRunLoopStop(CFRunLoopGetCurrent());
}

- (void)locationManager:(CLLocationManager *)manager didFailWithError:(NSError *)error {
  // To suppress annoying "unused variable" warning;
  (void)manager;
  (void)error;

  CFRunLoopStop(CFRunLoopGetCurrent());
}
@end

@implementation LocationService
- (void)run
{
  if (![CLLocationManager locationServicesEnabled]) {
    self.errorCode = 1;
    return;
  }

  id delegate = [[LocationDelegate alloc] init];
  CLLocationManager* locationManager = [[CLLocationManager alloc] init];
  [locationManager setDelegate:delegate];
  [locationManager requestLocation];
  CFRunLoopRun();


  CLLocation* loc = [locationManager location];

  NSTimeInterval interval = [[loc timestamp] timeIntervalSinceNow];

  double duration = -1 * @(interval).intValue;

  if (0 != duration) {
    self.errorCode = 3;
    self.errorDuration = duration;
  }

  // Simple heuristic for Error condition
  if (loc.horizontalAccuracy == 0.0 && loc.verticalAccuracy == 0.0) {
    self.errorCode = 2;
    return;
  }

  [locationManager release];
  [delegate release];

  self.latitude = loc.coordinate.latitude;
  self.longitude =  loc.coordinate.longitude;
  self.altitude = loc.altitude;
  self.horizontalAccuracy = loc.horizontalAccuracy;
  self.verticalAccuracy = loc.verticalAccuracy;
}
@end
