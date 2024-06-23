# ArchwayID Manager

## Key Functionalities

- Automatic Domain Renewal. Automatically extend your domains before they expire. The smart contract schedules the renewal to occur several days before the expiration date. Users have the option to cancel the renewal and receive a refund at any time.
- Set Default Domain. If you own multiple domains, you can easily select one as your default domain.
- Domain Renewal. Renew your domain directly with ease.

## How It Works

The ArchID auto-renewal functionality relies on the callback module. The admin of the smart contract can start a "cron" job that executes approximately every 7 days by registering a callback with the callback module. Upon execution, this "cron" job performs two tasks. First, it triggers the next callback to be executed in the next 7 days. Second, it checks whether any ArchID is going to expire in the upcoming 7 days. If it finds any, it triggers another callback to renew the domain. This process repeats until there are no more expiring domains in the list. This cycle continues indefinitely until the admin stops it or the contract runs out of funds.