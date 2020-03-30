#import <CoreLocation/CoreLocation.h>
#import <cocoa/cocoa.h>

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


NSArray* run()
{
  if (![CLLocationManager locationServicesEnabled]) {
    return nil;
  }

  id delegate = [[LocationDelegate alloc] init];
  CLLocationManager* locationManager = [[CLLocationManager alloc] init];
  [locationManager setDelegate:delegate];
  [locationManager requestLocation];
  CFRunLoopRun();


  CLLocation* loc = [locationManager location];

  NSTimeInterval interval = [[loc timestamp] timeIntervalSinceNow];

  if (0 != @(interval).intValue) {
    NSLog(@"Error condition: timestamp is %f seconds old", -1.0 * interval);
    // Continue for now
  }

  // Simple heuristic for Error condition
  if (loc.horizontalAccuracy == 0.0 && loc.verticalAccuracy == 0.0) {
    return nil;
  }

  [locationManager release];
  [delegate release];

  NSArray *result = [NSArray arrayWithObjects:
    [NSNumber numberWithDouble: loc.coordinate.latitude],
    [NSNumber numberWithDouble: loc.coordinate.longitude],
    [NSNumber numberWithDouble: loc.altitude],
    [NSNumber numberWithDouble: loc.horizontalAccuracy],
    [NSNumber numberWithDouble: loc.verticalAccuracy],
    nil
  ];

  return result;
}
