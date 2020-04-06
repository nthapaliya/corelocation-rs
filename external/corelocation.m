#import <CoreLocation/CoreLocation.h>
#import <cocoa/cocoa.h>

#include "corelocation.h"

@interface LocationDelegate : NSObject
- (void)locationManager:(CLLocationManager *)manager
     didUpdateLocations:(NSArray<CLLocation *> *)locations;
- (void)locationManager:(CLLocationManager *)manager didFailWithError:(NSError *)error;
@end

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

LocInfo run(void) {
  // Initialize with default values;
  struct LocInfo l = { };

  if (![CLLocationManager locationServicesEnabled]) {
    l.status = NOT_ENABLED;
    return l;
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
    l.status = STALE;
    l.error_duration = duration;
  }

  // Simple heuristic for Error condition
  if (loc.horizontalAccuracy == 0.0 && loc.verticalAccuracy == 0.0) {
    l.status = NOT_RETURNED;
    return l;
  }

  [locationManager release];
  [delegate release];

  l.latitude = loc.coordinate.latitude;
  l.longitude =  loc.coordinate.longitude;
  l.altitude = loc.altitude;
  l.h_accuracy = loc.horizontalAccuracy;
  l.v_accuracy = loc.verticalAccuracy;

  return l;
}
