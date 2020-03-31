#import <CoreLocation/CoreLocation.h>
#import <cocoa/cocoa.h>

@interface LocationDelegate : NSObject
- (void)locationManager:(CLLocationManager *)manager
     didUpdateLocations:(NSArray<CLLocation *> *)locations;
- (void)locationManager:(CLLocationManager *)manager didFailWithError:(NSError *)error;
@end

@interface LocationService : NSObject
- (void)run;

@property double latitude, longitude;
@property int altitude, horizontalAccuracy, verticalAccuracy;
@property int errorCode, errorDuration;
@end
